//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta371 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1409;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1410;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1411;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1412;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1413;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1414;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta371<F: Float>(t1569: F, t2880: F, t2862: F, t4437: F, t2888: F, t4433: F, t931: F, t10813: F, t1568: F, t4472: F, t950: F, t1581: F, t2924: F, t2906: F, t4475: F, t2932: F, t4471: F, t10747: F, t10771: F, t10811: F, t10825: F, t10828: F, t2861: F, t2886: F, t2905: F, t2930: F, t4454: F, t4476: F, t14279: F, t14373: F, t14428: F, t300: F, t4446: F, t961: F, t2948: F, t4483: F, t14364: F, t2907: F, t4496: F, t959: F, t2952: F, t10623: F, t1589: F, t14257: F, t14262: F, t14376: F, t14378: F, t14381: F, t14384: F, t14387: F, t14391: F, t14394: F, t14398: F, t14424: F, t14238: F, t360: F, t1021: F, t248: F, t3053: F, t4644: F, t10422: F, t4578: F, t3070: F, t1603: F, t3030: F, t3032: F, t3129: F, t3038: F, t1020: F, t10937: F, t10962: F, t10982: F, t10985: F, t10994: F, t11003: F, t14235: F, t1618: F, t3043: F, t3057: F, t3064: F, t3114: F, t3123: F, t3134: F, t4579: F, t4641: F, t4652: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t14429, t14432, t14436, t14439, t14443, t14450, t14453) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1409::<F>(t1569, t2880, t2862, t4437, t2888, t4433, t931, t10813, t1568, t4472, t950, t1581, t2924);
        let t14469 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1410::<F>(t2906, t4475, t2932, t4471, t950, t1581, t1569, t2862, t10747, t10771, t10811, t10825, t10828, t14429, t14432, t14436, t14439, t14443, t14450, t14453, t2861, t2886, t2905, t2930, t4454, t4476);
        let (t14472, t14475, t14477, t14479, t14480) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1411::<F>(t14279, t14373, t14428, t14469, t300, t4446, t961, t2948, t4483, t14364, t2907, t4496);
        let (t14482, t14484, t14486, t14487) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1412::<F>(t14480, t959, t2952, t4483, t10623, t1589, t14257, t14262, t14376, t14378, t14381, t14384, t14387, t14391, t14394, t14398, t14424, t14472, t14475, t14477, t14479);
        let (t14488, t14491, t14495, t14503, t14506) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1413::<F>(t14238, t14487, t360, t1021, t248, t3053, t4644, t10422, t4578, t3070, t1603, t3030);
        let t14523 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1414::<F>(t14506, t3032, t3129, t3038, t1020, t10937, t10962, t10982, t10985, t10994, t11003, t14235, t14491, t14495, t14503, t1618, t3043, t3057, t3064, t3070, t3114, t3123, t3134, t4579, t4641, t4644, t4652);
    (t14472, t14475, t14477, t14479, t14482, t14484, t14486, t14488, t14506, t14523)
}
