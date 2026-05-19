//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1442/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1442<F: Float>(t103218: F, t103699: F, t103879: F, t103927: F, t15245: F, t1734: F, t19201: F, t2148: F, t22034: F, t22040: F, t24849: F, t24851: F, t27406: F, t29702: F, t29709: F, t29750: F, t29787: F, t5398: F, t7283: F, t7376: F, t8067: F, t8070: F, t8083: F, t86000: F, t94395: F, t94858: F, t94963: F) -> F {
    let t109356 = -F::cast_from(0.82246703342411321826e-2_f64) * t103879 + t86000 - F::cast_from(0.24125699647107321069e0_f64) * t103218 * t8070 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t22040 * t2148 - F::new(3.0) * t15245 * t29709 + F::new(3.0) * t19201 * t8083 - F::cast_from(0.82246703342411321826e-2_f64) * t24849 * t24851 * t5398 * t1734 * t7376 + F::cast_from(0.16449340668482264365e-1_f64) * t94963 * t103699 - F::cast_from(0.80418998823691070229e-1_f64) * t103218 * t8067 - F::cast_from(0.13159472534785811492e0_f64) * t94858 * t29750 - F::cast_from(0.43864908449286038307e-1_f64) * t94395 * t29787 + F::cast_from(0.36554090374405031922e-2_f64) * t103927 + F::cast_from(0.65797362673929057459e-1_f64) * t27406 * t29702 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t22034 * t2148;
    t109356
}
