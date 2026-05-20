//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta610 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2196;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2197;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta610<F: Float>(t3540: F, t3567: F, t11159: F, t11539: F, t1174: F, t374: F, t485: F, t486: F, t9697: F, t1090: F, t3493: F, t11786: F, t3490: F, t11154: F, t11784: F, t1227: F, t248: F, t11814: F, t3572: F, t11825: F, t3523: F, t11820: F, t3536: F, t11778: F, t121: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t45224, t45227, t45250, t45251, t45256) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2196::<F>(t3540, t3567, t11159, t11539, t1174, t374, t485, t486, t9697, t1090, t3493, t11786, t3490);
        let (t45260, t45262, t45264, t45266, t45268) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2197::<F>(t11154, t11784, t1227, t248, t11814, t3572, t11825, t3523, t11820, t3536, t11778, t121);
    (t45224, t45227, t45250, t45251, t45256, t45260, t45262, t45264, t45266, t45268)
}
