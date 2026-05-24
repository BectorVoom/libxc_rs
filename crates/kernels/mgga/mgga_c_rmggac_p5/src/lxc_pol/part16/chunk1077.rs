//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1077/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1077<F: Float>(t39289: F, t42886: F, t42890: F, t42891: F, t42892: F, t42899: F, t42906: F, t45274: F, t45277: F, t45283: F, t45285: F, t45289: F, t45291: F, t45293: F, t45295: F, t45300: F, t45305: F, t45307: F) -> F {
    let t48407 = -F::cast_from(0.5107751987195740728e-4_f64) * t45274 + F::cast_from(0.212822999466489197e-4_f64) * t45277 + F::cast_from(0.1064114997332445985e-4_f64) * t45283 - F::cast_from(0.1702583995731913576e-4_f64) * t45285 - t42886 - t42890 + t42891 + F::cast_from(0.5454932330849068346e-1_f64) * t45289 - F::cast_from(0.40911992481368012595e-1_f64) * t45291 - t42892 + t42899 + F::cast_from(0.3405167991463827152e-4_f64) * t45293 - F::cast_from(0.5107751987195740728e-4_f64) * t45295 + t42906 - F::cast_from(0.79453919800822633544e-4_f64) * t39289 + F::cast_from(0.3192344991997337955e-4_f64) * t45300 + F::cast_from(0.1064114997332445985e-4_f64) * t45305 - F::cast_from(0.1064114997332445985e-4_f64) * t45307;
    t48407
}
