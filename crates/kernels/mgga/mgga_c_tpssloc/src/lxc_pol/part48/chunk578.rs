//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 578/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk578<F: Float>(t240: F, t6943: F, t1336: F, t1354: F, t1358: F, t2003: F, t552: F, t59: F, t1369: F, t6915: F, t6917: F, t6922: F, t6929: F, t6935: F, t6938: F, t6941: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6944 = t6943 * t240;
    let t6945 = t1336 * t6944;
    let t6946 = t6945 * t1354;
    let t6948 = t2003 * t1358;
    let t6949 = F::new(7.0) / F::new(2304.0) * t6948;
    let t6950 = t552 * t59;
    let t6951 = t6950 * t240;
    let t6952 = t1336 * t6951;
    let t6953 = t6952 * t1369;
    let t6955 = -t6915 - t6917 / F::new(48.0) - t6922 - F::cast_from(0.12111826828242117256e-2_f64) * t6929 - t6935 - F::cast_from(0.20186378047070195427e-3_f64) * t6938 + t6941 / F::new(1536.0) - t6946 / F::new(1536.0) - t6949 - t6953 / F::new(384.0);
    (t6944, t6945, t6946, t6948, t6950, t6951, t6952, t6953, t6955)
}
