

use super::graph::MpiGraph;


#[derive(Default)]
pub struct MpiAnalizator {
    // Calkowity ruch w sieci
    // suma od i=1 do i=N sum od k=1 do k=N po gamma_i_k
    //
    // czyli suma ruchu miedzy kazda para wezlow
    pub gamma_total_traffic: f64,
}


impl MpiAnalizator {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn analyze(&mut self, _graph: MpiGraph) {
        // ????????????
        self.gamma_total_traffic = 0.0; // ?????
    }
}