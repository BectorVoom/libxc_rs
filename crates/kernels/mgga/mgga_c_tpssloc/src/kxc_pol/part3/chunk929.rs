//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 929/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk929<F: Float>(t10922: F, t973: F, t2960: F, t3139: F, t1030: F, t363: F, t3068: F, t1058: F, t3030: F, t990: F, t3032: F, t3129: F) -> (F, F, F, F, F, F) {
    let t10923 = t973 * t10922;
    let t10927 = t2960 * t3139;
    let t10935 = t363 * t1030;
    let t10936 = t10935 * t3068;
    let t10937 = t1058 * t10936;
    let t10947 = t990 * t3030;
    let t10948 = t10947 * t3032;
    let t10949 = t10948 * t3129;
    (t10923, t10927, t10937, t10947, t10948, t10949)
}
