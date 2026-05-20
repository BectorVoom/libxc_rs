//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta585 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2317;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2318;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2319;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta585<F: Float>(t16132: F, t1825: F, t1352: F, t19743: F, t19660: F, t118: F, t6330: F, t794: F, t12202: F, t19631: F, t210: F, t214: F, t6347: F, t3739: F, t12211: F, t6353: F, t213: F, t1307: F, t221: F, t5187: F, t5196: F, t12188: F, t12190: F, t12194: F, t12196: F, t12200: F, t1315: F, t16101: F, t5195: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t19756, t19761, t19763, t19767, t19768, t19771) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2317::<F>(t16132, t1825, t1352, t19743, t19660, t118, t6330, t794, t12202, t19631, t210, t214);
        let (t19775, t19776, t19779, t19781, t19783, t19787) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2318::<F>(t118, t6347, t794, t3739, t12211, t6353, t213, t6330, t1307, t221, t5187, t5196);
        let t19790 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2319::<F>(t12188, t12190, t12194, t12196, t12200, t1315, t16101, t19768, t19771, t19776, t19779, t19783, t19787, t5195);
    (t19756, t19761, t19763, t19767, t19771, t19775, t19781, t19783, t19787, t19790)
}
