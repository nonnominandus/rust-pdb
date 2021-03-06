#![allow(dead_code)]
use crate::structs::*;
use crate::transformation::*;
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq)]
/// A Model containing multiple Chains
pub struct Model {
    /// The serial number of this Model
    serial_number: usize,
    /// The Chains making up this model
    chains: Vec<Chain>,
}

impl Model {
    /// Create a new Model
    ///
    /// ## Arguments
    /// * `serial_number` - the serial number
    pub fn new(serial_number: usize) -> Model {
        Model {
            serial_number,
            chains: Vec::new(),
        }
    }

    /// The serial number of this Model
    pub fn serial_number(&self) -> usize {
        self.serial_number
    }

    /// Set the serial number of this Model
    pub fn set_serial_number(&mut self, new_number: usize) {
        self.serial_number = new_number;
    }

    /// Get the amount of Chains making up this Model.
    pub fn chain_count(&self) -> usize {
        self.chains.len()
    }

    /// Get the amount of Residues making up this Model.
    pub fn residue_count(&self) -> usize {
        self.chains()
            .fold(0, |sum, chain| chain.residue_count() + sum)
    }

    /// Get the amount of Conformers making up this Model.
    pub fn conformer_count(&self) -> usize {
        self.chains()
            .fold(0, |sum, chain| chain.conformer_count() + sum)
    }

    /// Get the amount of Atoms making up this Model.
    pub fn atom_count(&self) -> usize {
        self.chains().fold(0, |sum, chain| chain.atom_count() + sum)
    }

    /// Get a specific Chain from list of Chains making up this Model.
    ///
    /// ## Arguments
    /// * `index` - the index of the Chain
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn chain(&self, index: usize) -> Option<&Chain> {
        self.chains().nth(index)
    }

    /// Get a specific Chain as a mutable reference from list of Chains making up this Model.
    ///
    /// ## Arguments
    /// * `index` - the index of the Chain
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn chain_mut(&mut self, index: usize) -> Option<&mut Chain> {
        self.chains_mut().nth(index)
    }

    /// Get a specific Residue from the Residues making up this Model.
    ///
    /// ## Arguments
    /// * `index` - the index of the Residue
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn residue(&self, index: usize) -> Option<&Residue> {
        self.residues().nth(index)
    }

    /// Get a specific Residue as a mutable reference from the Residues making up this Model.
    ///
    /// ## Arguments
    /// * `index` - the index of the Residue
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn residue_mut(&mut self, index: usize) -> Option<&mut Residue> {
        self.residues_mut().nth(index)
    }

    /// Get a specific Conformer from the Conformers making up this Model.
    ///
    /// ## Arguments
    /// * `index` - the index of the Conformer
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn conformer(&self, index: usize) -> Option<&Conformer> {
        self.conformers().nth(index)
    }

    /// Get a specific Conformer as a mutable reference from the Conformers making up this Model.
    ///
    /// ## Arguments
    /// * `index` - the index of the Conformer
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn conformer_mut(&mut self, index: usize) -> Option<&mut Conformer> {
        self.conformers_mut().nth(index)
    }

    /// Get a specific Atom from the Atoms making up this Model.
    ///
    /// ## Arguments
    /// * `index` - the index of the Atom
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn atom(&self, index: usize) -> Option<&Atom> {
        self.atoms().nth(index)
    }

    /// Get a specific Atom as a mutable reference from the Atoms making up this Model.
    ///
    /// ## Arguments
    /// * `index` - the index of the Atom
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn atom_mut(&mut self, index: usize) -> Option<&mut Atom> {
        self.atoms_mut().nth(index)
    }

    /// Get the list of Chains making up this Model.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn chains(&self) -> impl DoubleEndedIterator<Item = &Chain> + '_ {
        self.chains.iter()
    }

    /// Get the list of Chains as mutable references making up this Model.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn chains_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Chain> + '_ {
        self.chains.iter_mut()
    }

    /// Get the list of Residues making up this Model.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn residues(&self) -> impl DoubleEndedIterator<Item = &Residue> + '_ {
        self.chains.iter().flat_map(|a| a.residues())
    }

    /// Get the list of Residues as mutable references making up this Model.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn residues_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Residue> + '_ {
        self.chains.iter_mut().flat_map(|a| a.residues_mut())
    }

    /// Get the list of Conformers making up this Model.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn conformers(&self) -> impl DoubleEndedIterator<Item = &Conformer> + '_ {
        self.chains.iter().flat_map(|a| a.conformers())
    }

    /// Get the list of Conformers as mutable references making up this Model.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn conformers_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Conformer> + '_ {
        self.chains.iter_mut().flat_map(|a| a.conformers_mut())
    }

    /// Get the list of Atoms making up this Model.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn atoms(&self) -> impl DoubleEndedIterator<Item = &Atom> + '_ {
        self.chains.iter().flat_map(|a| a.atoms())
    }

    /// Get the list of Atoms as mutable references making up this Model.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn atoms_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Atom> + '_ {
        self.chains.iter_mut().flat_map(|a| a.atoms_mut())
    }

    /// Add a new Atom to this Model. It finds if there already is a Chain with the given `chain_id` if there is it will add this atom to that Chain, otherwise it will create a new Chain and add that to the list of Chains making up this Model. It does the same for the Residue, so it will create a new one if there does not yet exist a Residue with the given serial number.
    ///
    /// ## Arguments
    /// * `new_atom` - the new Atom to add
    /// * `chain_id` - the id of the Chain to add the Atom to
    /// * `residue_id` - the id construct of the Residue to add the Atom to
    /// * `conformer_id` - the id construct of the Conformer to add the Atom to
    ///
    /// ## Panics
    /// It panics if the Chain id or Residue name contains any invalid characters.
    pub fn add_atom(
        &mut self,
        new_atom: Atom,
        chain_id: &str,
        residue_id: (isize, Option<&str>),
        conformer_id: (&str, Option<&str>),
    ) {
        let mut found = false;
        let mut new_chain = Chain::new(chain_id).expect("Invalid characters in chain creation");
        let mut current_chain = &mut new_chain;
        for chain in &mut self.chains {
            if chain.id() == chain_id {
                current_chain = chain;
                found = true;
                break;
            }
        }
        #[allow(clippy::unwrap_used)]
        if !found {
            // As this moves the chain the atom should be added later to keep the reference intact
            self.chains.push(new_chain);
            current_chain = (&mut self.chains).last_mut().unwrap();
        }

        current_chain.add_atom(new_atom, residue_id, conformer_id);
    }

    /// Add a Chain to the list of Chains making up this Model. This does not detect any duplicates of names or serial numbers in the list of Chains.
    pub fn add_chain(&mut self, chain: Chain) {
        self.chains.push(chain);
    }

    /// Remove all Atoms matching the given predicate.
    /// As this is done in place this is the fastest way to remove Atoms from this Model.
    pub fn remove_atoms_by<F>(&mut self, predicate: F)
    where
        F: Fn(&Atom) -> bool,
    {
        for residue in self.residues_mut() {
            residue.remove_atoms_by(&predicate);
        }
    }

    /// Remove all Conformers matching the given predicate.
    /// As this is done in place this is the fastest way to remove Conformers from this Model.
    pub fn remove_conformers_by<F>(&mut self, predicate: F)
    where
        F: Fn(&Conformer) -> bool,
    {
        for chain in self.chains_mut() {
            chain.remove_conformers_by(&predicate);
        }
    }

    /// Remove all Residues matching the given predicate.
    /// As this is done in place this is the fastest way to remove Residues from this Model.
    pub fn remove_residues_by<F>(&mut self, predicate: F)
    where
        F: Fn(&Residue) -> bool,
    {
        for chain in self.chains_mut() {
            chain.remove_residues_by(&predicate);
        }
    }

    /// Remove all Chains matching the given predicate.
    /// As this is done in place this is the fastest way to remove Chains from this Model.
    pub fn remove_chains_by<F>(&mut self, predicate: F)
    where
        F: Fn(&Chain) -> bool,
    {
        self.chains.retain(|chain| !predicate(chain));
    }

    /// Remove the Chain specified.
    ///
    /// ## Arguments
    /// * `index` - the index of the Chain to remove
    ///
    /// ## Panics
    /// It panics when the index is outside bounds.
    pub fn remove_chain(&mut self, index: usize) -> Chain {
        self.chains.remove(index)
    }

    /// Remove the Chain specified. It returns `true` if it found a matching Chain and removed it.
    /// It removes the first matching Chain from the list.
    ///
    /// ## Arguments
    /// * `id` - the id of the Chain to remove
    pub fn remove_chain_by_id(&mut self, id: String) -> bool {
        let index = self.chains().position(|a| a.id() == id);

        if let Some(i) = index {
            self.remove_chain(i);
            true
        } else {
            false
        }
    }

    /// Remove all empty Chain from this Model, and all empty Residues from the Chains.
    pub fn remove_empty(&mut self) {
        self.chains.iter_mut().for_each(|c| c.remove_empty());
        self.chains.retain(|c| c.residue_count() > 0);
    }

    /// Apply a transformation to the position of all atoms making up this Model, the new position is immediately set.
    pub fn apply_transformation(&mut self, transformation: &TransformationMatrix) {
        for atom in self.atoms_mut() {
            atom.apply_transformation(transformation);
        }
    }

    /// Join this Model with another Model, this moves all atoms from the other Model
    /// to this Model. All other (meta) data of this Model will stay the same. It will add
    /// new Chains and Residues as defined in the other model.
    pub fn join(&mut self, other: Model) {
        self.chains.extend(other.chains);
    }

    /// Extend the Chains on this Model by the given iterator.
    pub fn extend<T: IntoIterator<Item = Chain>>(&mut self, iter: T) {
        self.chains.extend(iter);
    }
}

use std::fmt;
impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MODEL SerialNumber:{}, Chains: {}",
            self.serial_number,
            self.chains.len()
        )
    }
}

impl PartialOrd for Model {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.serial_number.cmp(&other.serial_number))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equality() {
        let a = Model::new(0);
        let b = a.clone();
        let c = Model::new(1);
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_ne!(b, c);
        assert!(a < c);
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_children() {
        let mut a = Model::new(0);
        a.add_chain(Chain::new("A").unwrap());
        let mut iter = a.chains();
        assert_eq!(iter.next(), Some(&Chain::new("A").unwrap()));
        assert_eq!(iter.next(), None);
        assert_eq!(a.chain_count(), 1);
        assert_eq!(a.conformer_count(), 0);
        assert_eq!(a.residue_count(), 0);
        assert_eq!(a.atom_count(), 0);
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_remove() {
        let mut a = Model::new(0);
        a.add_chain(Chain::new("A").unwrap());
        a.add_chain(Chain::new("C").unwrap());
        assert_eq!(a.remove_chain(0), Chain::new("A").unwrap());
        a.remove_chain_by_id("C".to_string());
        assert_eq!(a.chain_count(), 0);
    }

    #[test]
    fn test_display() {
        let a = Model::new(0);
        format!("{:?}", a);
        format!("{}", a);
    }
}
