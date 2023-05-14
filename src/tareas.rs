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
            id: self.id,
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
    fn ordenar_por_id(&mut self) -> ();
    fn siguente_id_disp(&self) -> i32;
    fn id_disponible(&self, id: i32) -> bool;
    fn buscar_id(&mut self, id: i32) -> Option<&mut Tarea>;
    fn buscar_nombre(&mut self, nombre: String) -> Vec<&mut Tarea>;
    fn buscar_descripcion(&mut self, descripcion: String) -> Vec<&mut Tarea>;
    fn buscar_estados(&mut self, estados: &Vec<Estado>) -> Vec<&mut Tarea>;
}

impl IdTareas for Vec<Tarea> {
    fn ordenar_por_id(&mut self) -> () {
        self.sort_by_key(|key| key.id);
    }

    fn siguente_id_disp(& self) -> i32 {
        if self.len() == 0 {
            return 1;
        }

        let mut id = 1;
        let mut tareas = self.clone();
        tareas.ordenar_por_id();

        for tarea in tareas.iter() {
            if tarea.id == id {
                id += 1;
            } else {
                return id;
            }
        }

        return id;
    }

    fn id_disponible(&self, id: i32) -> bool {
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

    fn buscar_id(&mut self, id: i32) -> Option<&mut Tarea> {
        return self.into_iter().find(|tarea| tarea.id == id);
    }

    fn buscar_nombre (&mut self, nombre: String) -> Vec<&mut Tarea> {
        return self
            .into_iter()
            .filter(|tarea| {
                return tarea.nombre.contains(nombre.as_str());
            })
            .collect();
    }

    fn buscar_descripcion (&mut self, descripcion: String) -> Vec<&mut Tarea> {
        return self
            .into_iter()
            .filter(|tarea| {
                return tarea.descripcion.contains(descripcion.as_str());
            })
            .collect();
    }

    fn buscar_estados (&mut self, estados: &Vec<Estado>) -> Vec<&mut Tarea> {
        println!("El valor de los estados es {:?}", estados);
        
        let tareas_filtradas: Vec<&mut Tarea> = self
            .iter_mut()
            .filter(|t| estados.contains(&t.estado))
            .collect();
        return tareas_filtradas;
    }
    
}
