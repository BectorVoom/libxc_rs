//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2411/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2411<F: Float>(t41662: F, t41675: F, t41678: F, t41682: F, t41684: F, t41863: F, t41865: F, t41870: F, t41872: F, t41874: F, t41876: F, t48982: F) -> F {
    let t49345 = F::cast_from(0.17215833333333333333e0_f64) * t41662 + F::cast_from(0.13772666666666666666e1_f64) * t41675 - F::cast_from(0.68863333333333333332e0_f64) * t41678 + F::new(0.103295e1) * t41682 + F::cast_from(0.16068111111111111111e1_f64) * t41684 + F::new(0.6311625e0) * t48982 + F::cast_from(0.92617777777777777776e0_f64) * t41863 - F::cast_from(0.13892666666666666667e0_f64) * t41865 - F::cast_from(0.34731666666666666666e0_f64) * t41870 - F::cast_from(0.11577222222222222222e0_f64) * t41872 + F::cast_from(0.69463333333333333333e-1_f64) * t41874 + F::cast_from(0.30872592592592592592e-1_f64) * t41876;
    t49345
}
