//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 851/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk851<F: Float>(t38872: F, t7487: F, t8466: F, t35207: F, t8469: F, t1591: F, t2046: F, t2050: F, t31: F, t34799: F, t34803: F, t38833: F, t38838: F, t38841: F, t38846: F, t38850: F, t38854: F, t38858: F, t38861: F, t38864: F, t38866: F, t38870: F) -> F {
    let t38873 = F::cast_from(0.10248087766267884742e-3_f64) * t38872;
    let t38874 = t7487 * t8466;
    let t38876 = t35207 * t8469;
    let t38881 = t2046 * t2050 * t1591 * t31;
    let t38882 = F::cast_from(0.43368970657079495312e-4_f64) * t38881;
    let t38883 = -F::cast_from(0.14408463291498358381e-2_f64) * t34799 + F::cast_from(0.30487649791575028314e-3_f64) * t38833 + t38838 - F::cast_from(0.43368970657079495312e-4_f64) * t38841 - F::cast_from(0.43368970657079495312e-4_f64) * t38846 - F::cast_from(0.72042316457491791906e-3_f64) * t38850 - t38854 + t38858 + t38861 + t38864 + F::cast_from(0.36021158228745895953e-3_f64) * t38866 - t38870 - t38873 - F::cast_from(0.19211284388664477842e-2_f64) * t38874 + F::cast_from(0.46116394948205481339e-3_f64) * t38876 - F::cast_from(0.2666855806192877858e0_f64) * t34803 + t38882;
    t38883
}
