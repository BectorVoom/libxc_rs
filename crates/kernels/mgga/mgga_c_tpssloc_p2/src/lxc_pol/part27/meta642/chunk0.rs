//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2185/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2185<F: Float>(t214: F, t4265: F, t1880: F, t6572: F, t25055: F, t81591: F, t13049: F, t13065: F, t13072: F, t13461: F, t1492: F, t22975: F, t23150: F, t25168: F, t25170: F, t259: F, t4268: F, t6627: F, t6663: F, t82154: F, t82172: F, t82174: F, t82182: F, t866: F, t87746: F, t87748: F, t87754: F, t87755: F, t87758: F, t87765: F, t87773: F, t87777: F, t87779: F) -> (F, F) {
    let t87782 = t214 * t4265;
    let t87784 = t1880 * t87782 * t6572;
    let t87786 = t81591 * t25055;
    let t87787 = F::cast_from(0.76763589786250567036e-1_f64) * t87786;
    let t87792 = -F::cast_from(0.82246703342411321825e-2_f64) * t87746 - t82154 + F::new(24.0) * t25168 * t87748 * t13049 - t87754 - F::new(12.0) * t87755 * t25170 - F::new(2.0) * t87758 * t866 + F::new(2.0) * t4268 * t22975 - F::cast_from(0.19739208802178717238e0_f64) * t87765 + t1492 * t23150 * t259 + F::cast_from(0.82246703342411321824e-2_f64) * t82172 - t6627 * t13461 + F::cast_from(0.76763589786250567036e-1_f64) * t82174 - F::cast_from(0.82246703342411321825e-2_f64) * t87773 + t87777 + F::cast_from(0.82246703342411321824e-2_f64) * t87779 - F::cast_from(0.82246703342411321824e-2_f64) * t82182 - F::cast_from(0.16449340668482264365e-1_f64) * t87784 - t87787 + F::new(4.0) * t6627 * t13072 - F::new(2.0) * t13065 * t6663;
    (t87782, t87792)
}
