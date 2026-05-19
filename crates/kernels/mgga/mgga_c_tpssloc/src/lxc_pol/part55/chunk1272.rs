//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1272/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1272<F: Float>(t24574: F, t34241: F, t27381: F, t8866: F, t7299: F, t8054: F, t1090: F, t117840: F, t118034: F, t1186: F, t14972: F, t15820: F, t1716: F, t2128: F, t2154: F, t24589: F, t24590: F, t27549: F, t27784: F, t3242: F, t3247: F, t32493: F, t32503: F, t32523: F, t34237: F, t34322: F, t3961: F, t45349: F, t4945: F, t5059: F, t7283: F, t7302: F, t8014: F, t8060: F, t8887: F, t8888: F, t8898: F, t94369: F, t94378: F, t94514: F) -> F {
    let t125206 = t24574 * t34241;
    let t125209 = t8866 * t27381;
    let t125218 = t7299 * t8054;
    let t125237 = -F::cast_from(0.3289868133696452873e-1_f64) * t2128 * t24590 * t34322 + F::cast_from(0.10966227112321509577e-1_f64) * t24589 * t94369 * t2154 * t3247 * t3961 + F::new(4.0) * t4945 * t32493 - F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t1716 * t32503 + F::new(2.0) * t15820 * t8888 + F::new(2.0) * t14972 * t8888 + F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t1716 * t118034 - F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t117840 * t8014 - F::cast_from(0.54831135561607547883e-2_f64) * t125206 - t15820 * t8898 + F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t1186 * t125209 - t14972 * t8898 + F::new(24.0) * t27784 * t45349 * t8887 * t5059 - F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t125218 * t7302 - F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t1186 * t34237 - F::cast_from(0.10966227112321509577e-1_f64) * t24589 * t94378 * t8060 * t1090 - F::cast_from(0.54831135561607547883e-2_f64) * t24589 * t94514 * t32523 - F::cast_from(0.73108180748810063844e-2_f64) * t27549 * t94369 * t2154 * t3242 * t3961;
    t125237
}
