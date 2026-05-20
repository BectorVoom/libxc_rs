//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta344 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1301;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1302;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1303;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta344<F: Float>(t9864: F, t9866: F, t3966: F, t751: F, t707: F, t2379: F, t262: F, t157: F, t9897: F, t2244: F, t4195: F, t2371: F, t4199: F, t40: F, t1409: F, t2517: F, t75: F, t12606: F, t1430: F, t2250: F, t4104: F, t607: F, t767: F, zeta_threshold: F, t52: F, t78: F, t1431: F, t4111: F, t771: F) -> (F, F, F, F, F, F, F, F) {
        let (t12927, t12928, t12934, t12935, t12942, t12943) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1301::<F>(t9864, t9866, t3966, t751, t707, t2379, t262, t157, t9897, t2244, t4195, t2371, t4199);
        let (t12944, t12947, t12958) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1302::<F>(t40, t12943, t1409, t2517, t707, t3966, t75, t12606, t1430, t2244, t2250, t4104, t607, t767, zeta_threshold);
        let t12971 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1303::<F>(t52, t3966, t78, t12606, t1431, t2244, t2250, t4111, t607, t771, t12958, zeta_threshold);
    (t12927, t12928, t12934, t12935, t12942, t12944, t12947, t12971)
}
