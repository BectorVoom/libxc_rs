//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 971/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk971<F: Float>(t2989: F, t5398: F, t2988: F, t10186: F, t13830: F, t13850: F, t17770: F, t17773: F, t17778: F, t17784: F, t17788: F, t17791: F, t2960: F, t2986: F, t5818: F, t5821: F, t5829: F, t973: F) -> (F,) {
    let t17794 = t2989 * t5398;
    let t17795 = t2988 * t17794;
    let t17798 = t13830 - 0.74074074074074074072e-3 * t2960 * t5829 + 0.9259259259259259259e-4 * t17770 + 0.27777777777777777777e-3 * t973 * t17773 - 0.83333333333333333332e-3 * t973 * t17778 - 0.98765432098765432096e-3 * t2960 * t5818 + 0.12345679012345679012e-3 * t17784 + 0.14814814814814814814e-2 * t10186 * t5821 - 0.55555555555555555554e-3 * t2986 * t17788 - t13850 + 0.37037037037037037036e-3 * t2986 * t17791 - 0.27777777777777777777e-3 * t2986 * t17795;
    (t17798,)
}
