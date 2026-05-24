//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1035/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1035<F: Float>(t38382: F, t38414: F, t38363: F, t38365: F, t38367: F, t38371: F, t38375: F, t38377: F, t38387: F, t38389: F, t38391: F, t38393: F, t38395: F, t38398: F, t38404: F, t38412: F, t38420: F) -> F {
    let t42600 = F::cast_from(0.2927036860455597649e0_f64) * t38382;
    let t42609 = F::cast_from(0.39726959900411316772e-4_f64) * t38414;
    let t42611 = F::cast_from(0.5107751987195740728e-4_f64) * t38363 - F::cast_from(0.85129199786595678799e-5_f64) * t38365 - F::cast_from(0.1702583995731913576e-4_f64) * t38367 - F::cast_from(0.212822999466489197e-4_f64) * t38371 - F::cast_from(0.212822999466489197e-4_f64) * t38375 - F::cast_from(0.1064114997332445985e-4_f64) * t38377 + t42600 + F::cast_from(0.1702583995731913576e-4_f64) * t38387 + F::cast_from(0.1702583995731913576e-4_f64) * t38389 - F::cast_from(0.5107751987195740728e-4_f64) * t38391 + F::cast_from(0.5107751987195740728e-4_f64) * t38393 + F::cast_from(0.1702583995731913576e-4_f64) * t38395 + F::cast_from(0.5107751987195740728e-4_f64) * t38398 + F::cast_from(0.2553875993597870364e-4_f64) * t38404 + F::cast_from(0.85129199786595678799e-5_f64) * t38412 + t42609 + F::cast_from(0.15323255961587222184e-3_f64) * t38420;
    t42611
}
