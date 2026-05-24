//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 892/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk892<F: Float>(t34785: F, t34788: F, t34794: F, t34799: F, t38819: F, t38823: F, t38826: F, t38833: F, t38838: F, t38841: F, t38846: F, t38850: F, t38854: F, t38858: F, t38861: F, t38864: F, t5928: F, t8390: F) -> F {
    let t44972 = -F::cast_from(0.23948483403727617128e0_f64) * t5928 * t8390 + t38819 - t38823 + F::cast_from(0.60975299583150056628e-3_f64) * t38826 - t34785 + t34788 - t34794 - F::cast_from(0.72042316457491791906e-3_f64) * t34799 + F::cast_from(0.60975299583150056628e-3_f64) * t38833 + t38838 - F::cast_from(0.86737941314158990623e-4_f64) * t38841 - F::cast_from(0.86737941314158990623e-4_f64) * t38846 - F::cast_from(0.14408463291498358381e-2_f64) * t38850 - t38854 + t38858 + t38861 + t38864;
    t44972
}
