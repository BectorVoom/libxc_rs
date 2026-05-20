//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta382 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1572;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1573;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1574;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta382<F: Float>(t14238: F, t14487: F, t360: F, t1021: F, t248: F, t3053: F, t4644: F, t10422: F, t4578: F, t3070: F, t1603: F, t3030: F, t3032: F, t3129: F, t3038: F, t1020: F, t10937: F, t10962: F, t10982: F, t10985: F, t10994: F, t11003: F, t14235: F, t1618: F, t3043: F, t3057: F, t3064: F, t3114: F, t3123: F, t3134: F, t4579: F, t4641: F, t4652: F, t13953: F, t14004: F, t14050: F, t14074: F, t14120: F, t14170: F, t14233: F, t349: F, t225: F, t4658: F, t1625: F, t3020: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t14488, t14491, t14495, t14501, t14503, t14506) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1572::<F>(t14238, t14487, t360, t1021, t248, t3053, t4644, t10422, t4578, t3070, t1603, t3030);
        let (t14507, t14523) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1573::<F>(t14506, t3032, t3129, t3038, t1020, t10937, t10962, t10982, t10985, t10994, t11003, t14235, t14491, t14495, t14503, t1618, t3043, t3057, t3064, t3070, t3114, t3123, t3134, t4579, t4641, t4644, t4652);
        let (t14526, t14527, t14529, t14532) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1574::<F>(t13953, t14004, t14050, t14074, t14120, t14170, t14233, t14523, t349, t225, t4658, t1625, t3020);
    (t14488, t14491, t14501, t14506, t14507, t14526, t14527, t14529, t14532)
}
