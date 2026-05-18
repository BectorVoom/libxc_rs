//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1300/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1300<F: Float>(t113131: F, t118436: F, t118439: F, t118440: F, t118455: F, t118465: F, t118467: F, t118949: F, t118954: F, t1877: F, t22959: F, t25: F, t25021: F, t25024: F, t25028: F, t2522: F, t25366: F, t25372: F, t25375: F, t25377: F, t25381: F, t25385: F, t25392: F, t30757: F, t30770: F, t32886: F, t606: F, t6542: F, t8366: F, t8370: F) -> F {
    let t118964 = F::new(3.0) / F::new(2.0) * t2522 * t32886 * t6542 - F::new(3.0) / F::new(2.0) * t2522 * t8370 * t25024 + t118436 * t25375 - F::new(3.0) * t118439 * t118440 + F::new(3.0) / F::new(2.0) * t2522 * t8366 * t25028 - t1877 * t30757 * t25381 / F::new(2.0) + F::new(3.0) / F::new(2.0) * t2522 * t8366 * t25385 + t1877 * t30770 * t25392 - F::new(3.0) * t22959 * t118455 + F::new(3.0) / F::new(2.0) * t2522 * t8366 * t25024 - t1877 * t30757 * t25392 / F::new(2.0) + t118465 - F::new(3.0) * t22959 * t118467 - F::new(3.0) / F::new(2.0) * t113131 * t25021 + t1877 * t118949 * t25 / F::new(2.0) + F::new(2.0) * t25372 * t118954 + t1877 * t32886 * t606 / F::new(2.0) + t1877 * t30770 * t25377 - F::new(3.0) / F::new(2.0) * t113131 * t25366;
    t118964
}
