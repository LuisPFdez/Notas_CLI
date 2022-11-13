#[derive(Debug, PartialEq, PartialOrd)]
pub enum Estado {
    PENDIENTE,
    ENPROCESO,
    FINALIZADO,
}

#[derive(Debug, PartialOrd)]
pub struct Tarea {
    pub id: i32,
    pub nombre: String,
    pub descripcion: String,
    pub estado: Estado,
}

impl Tarea {}

impl Estado {
    pub fn vec() -> Vec<&'static str> {
        return vec!["Pendiente", "En Proceso", "Finalizado"];
    }

    pub fn obtener_estado(pos: usize) -> Estado {
        match pos {
            0 => return Estado::PENDIENTE,
            1 => return Estado::ENPROCESO,
            2 => return Estado::FINALIZADO,
            _ => return Estado::PENDIENTE,
        }
    }
}

impl std::fmt::Display for Estado {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Eq for Tarea {}

impl Ord for Tarea {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialEq for Tarea {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Clone for Tarea {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            nombre: self.nombre.clone(),
            descripcion: self.descripcion.clone(),
            estado: self.estado.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}

impl Clone for Estado {
    fn clone(&self) -> Self {
        match self {
            Self::PENDIENTE => Self::PENDIENTE,
            Self::ENPROCESO => Self::ENPROCESO,
            Self::FINALIZADO => Self::FINALIZADO,
        }
    }

    fn clone_from(&mut self, source: &Self) {
        *self = source.clone();
    }
}

pub trait IdTareas {
    fn siguente_id_disp(&mut self) -> i32;
    fn id_disponible(&mut self, id: i32) -> bool;
}

impl IdTareas for Vec<Tarea> {
    fn siguente_id_disp(&mut self) -> i32 {
        if self.len() == 0 {
            return 1;
        }

        let mut id = 1;

        self.sort();
        for tarea in self.iter() {
            if tarea.id == id {
                id += 1;
            } else {
                return id;
            }
        }

        return id;
    }

    fn id_disponible(&mut self, id: i32) -> bool {
        if self.len() == 0 {
            return true;
        }

        for tarea in self.iter() {
            if tarea.id == id {
                return false;
            }
        }

        return true;
    }
}
