//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1193/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1193<F: Float>(t10575: F, t17964: F, t10606: F, t10813: F, t10790: F, t19703: F, t1381: F, t61050: F, t10587: F, t10610: F, t10614: F, t1369: F, t61062: F, t17974: F, t3689: F, t10623: F, t5559: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t63932 = t17964 * t10575;
    let t63939 = t17964 * t10606;
    let t63941 = t17964 * t10813;
    let t63943 = t19703 * t10790;
    let t63945 = t61050 * t1381;
    let t63951 = t19703 * t10587;
    let t63953 = t19703 * t10610;
    let t63955 = t19703 * t10614;
    let t63957 = t61062 * t1369;
    let t63960 = t17974 * t3689;
    let t63962 = t5559 * t10623;
    (t63932, t63939, t63941, t63943, t63945, t63951, t63953, t63955, t63957, t63960, t63962)
}
