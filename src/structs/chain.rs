#![allow(dead_code)]
use crate::structs::*;
use crate::transformation::*;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
/// A Chain containing multiple Residues
pub struct Chain {
    /// The identifier of this Chain
    id: String,
    /// The Residues making up this Chain
    residues: Vec<Residue>,
    /// A possible reference to a database for this chain
    database_reference: Option<DatabaseReference>,
}

impl Chain {
    /// Create a new Chain
    ///
    /// ## Arguments
    /// * `id` - the identifier
    ///
    /// ## Fails
    /// It fails if the identifier is an invalid character.
    pub fn new(id: &str) -> Option<Chain> {
        if !valid_identifier(&id) {
            return None;
        }
        Some(Chain {
            id: id.trim().to_ascii_uppercase(),
            residues: Vec::new(),
            database_reference: None,
        })
    }

    /// The ID of the Chain
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Set the ID of the Chain, returns `false` if the new id is an invalid character.
    /// The ID will be changed to uppercase as requested by PDB/PDBx standard.
    pub fn set_id(&mut self, new_id: &str) -> bool {
        if valid_identifier(&new_id) {
            self.id = new_id.trim().to_ascii_uppercase();
            true
        } else {
            false
        }
    }

    /// Get the database reference, if any, for this chain.
    pub fn database_reference(&self) -> Option<&DatabaseReference> {
        self.database_reference.as_ref()
    }

    /// Get the database reference mutably, if any, for this chain.
    pub fn database_reference_mut(&mut self) -> Option<&mut DatabaseReference> {
        self.database_reference.as_mut()
    }

    /// Set the database reference for this chain.
    pub fn set_database_reference(&mut self, reference: DatabaseReference) {
        self.database_reference = Some(reference);
    }

    /// Get the amount of Residues making up this Chain
    pub fn residue_count(&self) -> usize {
        self.residues.len()
    }

    /// Get the amount of Conformers making up this Chain
    pub fn conformer_count(&self) -> usize {
        self.residues()
            .fold(0, |sum, res| res.conformer_count() + sum)
    }

    /// Get the amount of Atoms making up this Chain
    pub fn atom_count(&self) -> usize {
        self.residues().fold(0, |sum, res| res.atom_count() + sum)
    }

    /// Get a specific Residue from list of Residues making up this Chain.
    ///
    /// ## Arguments
    /// * `index` - the index of the Residue
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn residue(&self, index: usize) -> Option<&Residue> {
        self.residues.get(index)
    }

    /// Get a specific Residue as a mutable reference from list of Residues making up this Chain.
    ///
    /// ## Arguments
    /// * `index` - the index of the Residue
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn residue_mut(&mut self, index: usize) -> Option<&mut Residue> {
        self.residues.get_mut(index)
    }

    /// Get a specific Conformer from list of Conformers making up this Chain.
    ///
    /// ## Arguments
    /// * `index` - the index of the Conformer
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn conformer(&self, index: usize) -> Option<&Conformer> {
        self.conformers().nth(index)
    }

    /// Get a specific Conformer as a mutable reference from list of Conformers making up this Chain.
    ///
    /// ## Arguments
    /// * `index` - the index of the Conformer
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn conformer_mut(&mut self, index: usize) -> Option<&mut Conformer> {
        self.conformers_mut().nth(index)
    }

    /// Get a specific Atom from the Atoms making up this Chain.
    ///
    /// ## Arguments
    /// * `index` - the index of the Atom
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn atom(&self, index: usize) -> Option<&Atom> {
        self.atoms().nth(index)
    }

    /// Get a specific Atom as a mutable reference from the Atoms making up this Chain.
    ///
    /// ## Arguments
    /// * `index` - the index of the Atom
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn atom_mut(&mut self, index: usize) -> Option<&mut Atom> {
        self.atoms_mut().nth(index)
    }

    /// Get the list of Residues making up this Chain.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn residues(&self) -> impl DoubleEndedIterator<Item = &Residue> + '_ {
        self.residues.iter()
    }

    /// Get the list of Residues as mutable references making up this Chain.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn residues_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Residue> + '_ {
        self.residues.iter_mut()
    }

    /// Get the list of Conformers making up this Chain.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn conformers(&self) -> impl DoubleEndedIterator<Item = &Conformer> + '_ {
        self.residues.iter().flat_map(|a| a.conformers())
    }

    /// Get the list of Conformers as mutable references making up this Chain.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn conformers_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Conformer> + '_ {
        self.residues.iter_mut().flat_map(|a| a.conformers_mut())
    }

    /// Get the list of Atoms making up this Chain.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn atoms(&self) -> impl DoubleEndedIterator<Item = &Atom> + '_ {
        self.residues.iter().flat_map(|a| a.atoms())
    }

    /// Get the list of Atoms as mutable references making up this Chain.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn atoms_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Atom> + '_ {
        self.residues.iter_mut().flat_map(|a| a.atoms_mut())
    }

    /// Add a new Atom to this Chain. It finds if there already is a Residue with the given serial number if there is it will add this atom to that Residue, otherwise it will create a new Residue and add that to the list of Residues making up this Chain.
    ///
    /// ## Arguments
    /// * `new_atom` - the new Atom to add
    /// * `residue_id` - the id construct of the Residue to add the Atom to
    /// * `conformer_id` - the id construct of the Conformer to add the Atom to
    ///
    /// ## Panics
    /// It panics if the Residue name contains any invalid characters.
    pub fn add_atom(
        &mut self,
        new_atom: Atom,
        residue_id: (isize, Option<&str>),
        conformer_id: (&str, Option<&str>),
    ) {
        let mut found = false;
        let mut new_residue = Residue::new(residue_id.0, residue_id.1, None)
            .expect("Invalid chars in Residue creation");
        let mut current_residue = &mut new_residue;
        for residue in &mut self.residues {
            if residue.id() == residue_id {
                current_residue = residue;
                found = true;
                break;
            }
        }
        #[allow(clippy::unwrap_used)]
        if !found {
            self.residues.push(new_residue);
            current_residue = self.residues.last_mut().unwrap();
        }

        current_residue.add_atom(new_atom, conformer_id);
    }

    /// Add a Residue end of to the list of Residues making up this Chain. This does not detect any duplicates of names or serial numbers in the list of Residues.
    pub fn add_residue(&mut self, residue: Residue) {
        self.residues.push(residue);
    }

    /// Inserts a Residue to the index in the list of Residues making up this Chain. This does not detect any duplicates of names or serial numbers in the list of Residues.
    /// This panics if `index > len`.
    pub fn insert_residue(&mut self, index: usize, residue: Residue) {
        self.residues.insert(index, residue);
    }

    /// Remove all Atoms matching the given predicate. As this is done in place this is the fastest way to remove Atoms from this Chain.
    pub fn remove_atoms_by<F>(&mut self, predicate: F)
    where
        F: Fn(&Atom) -> bool,
    {
        for residue in self.residues_mut() {
            residue.remove_atoms_by(&predicate);
        }
    }

    /// Remove all Conformers matching the given predicate. As this is done in place this is the fastest way to remove Conformers from this Chain.
    pub fn remove_conformers_by<F>(&mut self, predicate: F)
    where
        F: Fn(&Conformer) -> bool,
    {
        for residue in self.residues_mut() {
            residue.remove_conformers_by(&predicate);
        }
    }

    /// Remove all residues matching the given predicate. As this is done in place this is the fastest way to remove Residues from this Chain.
    pub fn remove_residues_by<F>(&mut self, predicate: F)
    where
        F: Fn(&Residue) -> bool,
    {
        self.residues.retain(|residue| !predicate(residue));
    }

    /// Remove the Residue specified.
    ///
    /// ## Arguments
    /// * `index` - the index of the Residue to remove
    ///
    /// ## Panics
    /// It panics when the index is outside bounds.
    pub fn remove_residue(&mut self, index: usize) {
        self.residues.remove(index);
    }

    /// Remove the Residue specified. It returns `true` if it found a matching Residue and removed it.
    /// It removes the first matching Residue from the list.
    ///
    /// ## Arguments
    /// * `id` - the id construct of the Residue to remove (see Residue.id())
    pub fn remove_residue_by_id(&mut self, id: (isize, Option<&str>)) -> bool {
        let index = self.residues.iter().position(|a| a.id() == id);

        if let Some(i) = index {
            self.remove_residue(i);
            true
        } else {
            false
        }
    }

    /// Remove all empty Residues from this Chain, and all empty Conformers from the Residues.
    pub fn remove_empty(&mut self) {
        self.residues.iter_mut().for_each(|r| r.remove_empty());
        self.residues.retain(|r| r.conformer_count() > 0);
    }

    /// Apply a transformation to the position of all atoms making up this Chain, the new position is immediately set.
    pub fn apply_transformation(&mut self, transformation: &TransformationMatrix) {
        for atom in self.atoms_mut() {
            atom.apply_transformation(transformation);
        }
    }

    /// Join this Chain with another Chain, this moves all atoms from the other Chain
    /// to this Chain. All other (meta) data of this Chain will stay the same.
    pub fn join(&mut self, other: Chain) {
        self.residues.extend(other.residues);
    }

    /// Extend the Residues on this Chain by the given iterator.
    pub fn extend<T: IntoIterator<Item = Residue>>(&mut self, iter: T) {
        self.residues.extend(iter);
    }
}

use std::fmt;
impl fmt::Display for Chain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CHAIN ID:{}, Residues: {}",
            self.id(),
            self.residues.len()
        )
    }
}

impl PartialEq for Chain {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id() && self.residues == other.residues
    }
}

impl PartialOrd for Chain {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.id().cmp(&other.id()))
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn test_id_validation() {
        let mut a = Chain::new("A").unwrap();
        assert_eq!(Chain::new("R̊"), None);
        assert!(!a.set_id("Oͦ"));
        assert_eq!(a.id(), "A");
        a.set_id("atom");
        assert_eq!(a.id(), "ATOM");
    }

    #[test]
    fn ordering_and_equality() {
        let a = Chain::new("A").unwrap();
        let b = Chain::new("A").unwrap();
        let c = Chain::new("B").unwrap();
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert!(a < c);
        assert!(b < c);
    }

    #[test]
    fn test_empty_chain() {
        let mut a = Chain::new("A").unwrap();
        assert_eq!(a.database_reference(), None);
        assert_eq!(a.database_reference_mut(), None);
        assert_eq!(a.residue_count(), 0);
        assert_eq!(a.conformer_count(), 0);
        assert_eq!(a.atom_count(), 0);
    }

    #[test]
    fn test_residue() {
        let mut a = Chain::new("A").unwrap();
        let mut r = Residue::new(1, None, None).unwrap();
        a.add_residue(r.clone());
        a.add_residue(Residue::new(13, None, None).unwrap());
        assert_eq!(a.residue(0), Some(&r));
        assert_eq!(a.residue_mut(0), Some(&mut r));
        a.remove_residue(0);
        assert!(a.remove_residue_by_id((13, None)));
        assert_eq!(a.residue_count(), 0);
        assert!(!a.remove_residue_by_id((13, None)));
    }

    #[test]
    fn check_display() {
        let a = Chain::new("A").unwrap();
        format!("{:?}", a);
        format!("{}", a);
    }
}
