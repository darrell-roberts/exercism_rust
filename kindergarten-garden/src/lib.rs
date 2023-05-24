#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
enum Student {
    Alice,
    Bob,
    Charlie,
    David,
    Eve,
    Fred,
    Ginny,
    Harriet,
    Ileana,
    Joseph,
    Kincaid,
    Larry,
}

#[derive(Debug)]
struct UnknownStudent;

impl TryFrom<&str> for Student {
    type Error = UnknownStudent;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let c = match value {
            "Alice" => Self::Alice,
            "Bob" => Self::Bob,
            "Charlie" => Self::Charlie,
            "David" => Self::David,
            "Eve" => Self::Eve,
            "Fred" => Self::Fred,
            "Ginny" => Self::Ginny,
            "Harriet" => Self::Harriet,
            "Ileana" => Self::Ileana,
            "Joseph" => Self::Joseph,
            "Kincaid" => Self::Kincaid,
            "Larry" => Self::Larry,
            _ => return Err(UnknownStudent),
        };
        Ok(c)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Plant {
    Grass,
    Clover,
    Radishes,
    Violets,
}

#[derive(Debug)]
struct UnknownPlant;

impl TryFrom<char> for Plant {
    type Error = UnknownPlant;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let p = match value {
            'G' => Self::Grass,
            'C' => Self::Clover,
            'R' => Self::Radishes,
            'V' => Self::Violets,
            _ => return Err(UnknownPlant),
        };
        Ok(p)
    }
}

impl From<&Plant> for &'static str {
    fn from(value: &Plant) -> Self {
        match value {
            Plant::Grass => "grass",
            Plant::Clover => "clover",
            Plant::Radishes => "radishes",
            Plant::Violets => "violets",
        }
    }
}

// Unfortunately Rust does not have an Enum Trait
// like Haskell. It is an open RFC though.
// https://github.com/rust-lang/rfcs/issues/284
const STUDENTS: [Student; 12] = [
    Student::Alice,
    Student::Bob,
    Student::Charlie,
    Student::David,
    Student::Eve,
    Student::Fred,
    Student::Ginny,
    Student::Harriet,
    Student::Ileana,
    Student::Joseph,
    Student::Kincaid,
    Student::Larry,
];

/// An Iterator that yields a student and two plants.
struct StudentPlantPairIter<P, S>
where
    P: Iterator<Item = Plant>,
    S: Iterator<Item = Student>,
{
    plant_iter: P,
    student_iter: S,
}

impl<P, S> Iterator for StudentPlantPairIter<P, S>
where
    P: Iterator<Item = Plant>,
    S: Iterator<Item = Student>,
{
    type Item = (Student, Plant, Plant);

    fn next(&mut self) -> Option<Self::Item> {
        let s = self.student_iter.next()?;
        let p1 = self.plant_iter.next()?;
        let p2 = self.plant_iter.next()?;

        Some((s, p1, p2))
    }
}

/// Create a student plant pair iterator.
fn student_plant_iter(row: &str) -> impl Iterator<Item = (Student, Plant, Plant)> + '_ {
    StudentPlantPairIter {
        plant_iter: row.chars().flat_map(|c| Plant::try_from(c).ok()),
        student_iter: STUDENTS.into_iter(),
    }
}

/// Parse the diagram and collect the 4 plants for provided student.
pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    student
        .try_into()
        .ok()
        .and_then(|student| {
            diagram.split_once('\n').and_then(|(row1, row2)| {
                student_plant_iter(row1)
                    .zip(student_plant_iter(row2))
                    .find_map(|((s1, p1, p2), (s2, p3, p4))| {
                        (s1 == s2 && s1 == student).then_some([p1, p2, p3, p4])
                    })
            })
        })
        .map(|plants| plants.iter().map(Into::into).collect::<Vec<_>>())
        .unwrap_or_default()
}
