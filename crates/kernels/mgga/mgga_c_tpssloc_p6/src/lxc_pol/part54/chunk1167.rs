//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1167/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1167<F: Float>(t25: F, t265: F, t394: F, t31477: F, t1877: F, t24191: F, t24339: F, t2522: F, t26756: F, t30767: F, t31430: F, t31434: F, t31442: F, t31449: F, t31451: F, t40: F, t606: F, t607: F, t6542: F, t6671: F, t7114: F, t8566: F, t8569: F, t8580: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t31478 = piecewise3::<F>(t395, F::new(0.0), t31477);
    let t31483 = piecewise3::<F>(t115, F::new(3.0) / F::new(2.0) * t2522 * t8566 * t6542 + t1877 * t31430 * t25 / F::new(2.0) - t1877 * t31434 * t6671 / F::new(2.0) + t1877 * t8566 * t606 / F::new(2.0) - F::new(3.0) / F::new(2.0) * t24191 * t31442 - t1877 * t24339 * t8569 / F::new(2.0) + t26756 * t31449 - t1877 * t7114 * t31451 / F::new(2.0) - t1877 * t7114 * t30767 / F::new(2.0), t31478 * t40 / F::new(2.0) + t8580 * t607 / F::new(2.0));
    (t31478, t31483)
}
