//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1968/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1968<F: Float>(t193: F, t7125: F, t26739: F, t2752: F, t200: F, t7109: F, t24191: F, t86755: F, t1877: F, t2057: F, t24335: F, t24339: F, t24344: F, t25015: F, t2522: F, t25375: F, t25377: F, t25381: F, t25392: F, t26563: F, t26756: F, t6671: F, t7114: F, t7475: F, t86764: F, t86794: F, t86806: F, t86810: F, t86830: F, t87957: F, t87961: F) -> (F, F, F, F, F) {
    let t92271 = t193 * t7125;
    let t92276 = t26739 * t2752;
    let t92295 = t193 * t200 * t7109;
    let t92299 = F::new(6.0) * t24191 * t86755;
    let t92309 = F::new(2.0) * t92271 * t25375 + F::new(2.0) * t26756 * t86794 - t1877 * t92276 * t6671 - t1877 * t24339 * t25392 + F::new(3.0) * t2522 * t2057 * t87957 + F::new(3.0) / F::new(2.0) * t2522 * t2057 * t86764 - t1877 * t7114 * t86806 / F::new(2.0) + F::new(6.0) * t26563 * t86830 - t1877 * t24339 * t25381 + F::new(6.0) * t92295 * t25015 + t92299 + t1877 * t24344 * t87961 + F::new(3.0) / F::new(2.0) * t2522 * t24335 * t7475 - t1877 * t24339 * t25377 - F::new(3.0) * t24191 * t86810;
    (t92271, t92276, t92295, t92299, t92309)
}
