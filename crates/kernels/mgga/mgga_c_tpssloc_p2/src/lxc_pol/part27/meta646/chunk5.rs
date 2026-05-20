//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2224/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2224<F: Float>(t225: F, t25820: F, t23384: F, t25827: F, t25436: F, t23328: F, t23394: F, t10170: F, t1049: F, t1050: F, t1066: F, t13735: F, t13743: F, t14549: F, t14659: F, t1634: F, t1635: F, t1956: F, t23327: F, t23331: F, t254: F, t25712: F, t25759: F, t343: F, t50703: F, t6687: F, t6690: F, t6704: F, t6771: F, t7625: F, t82481: F, t83276: F, t83281: F, t883: F) -> F {
    let t88744 = t25820 * t225;
    let t88753 = F::cast_from(0.54831135561607547884e-2_f64) * t23384 * t25827;
    let t88758 = F::cast_from(0.18277045187202515961e-2_f64) * t23384 * t25436;
    let t88772 = t23328 * t23394;
    let t88779 = -F::new(2.0) * t88744 * t1066 - t6771 * t14659 - F::cast_from(0.49348022005446793095e-1_f64) * t6687 * t6704 * t82481 * t13735 - t88753 - t50703 * t1956 + F::new(2.0) * t6771 * t14549 + t88758 - F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t25712 * t343 * t1049 * t6690 - F::new(12.0) * t1050 * t254 * t25759 + F::new(4.0) * t6771 * t13743 - t10170 * t7625 - F::new(2.0) * t83276 * t1635 + F::cast_from(0.10966227112321509577e-1_f64) * t23327 * t88772 * t1634 * t883 * t23331 - F::cast_from(0.12184696791468343974e-2_f64) * t83281;
    t88779
}
