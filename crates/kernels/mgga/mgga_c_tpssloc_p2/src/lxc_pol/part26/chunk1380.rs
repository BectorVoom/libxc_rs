//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1380/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1380<F: Float>(t265: F, t504: F, t24901: F, t3640: F, t11947: F, t7394: F, t2157: F, t43706: F, t11940: F, t11944: F, t1254: F, t1256: F, t193: F, t24905: F, t24909: F, t336: F, t3633: F, t3637: F, t4700: F, t51906: F, t7398: F, t83543: F, t85673: F, t85713: F, t85749: F, t85791: F, t86399: F, t86436: F, t86468: F, t86506: F) -> F {
    let t505 = t265 < t504;
    let t86513 = t24901 * t3640;
    let t86517 = t7394 * t11947;
    let t86524 = t2157 * t43706;
    let t86534 = piecewise3::<F>(t505, t193 * t336 * (t85673 + t85713 + t85749 + t85791 + t86399 + t86436 + t86468 + t86506) * t1256 - F::new(3.0) * t4700 * t86513 * t1254 + F::new(6.0) * t4700 * t86517 * t3637 - F::new(3.0) * t4700 * t24905 * t3633 - F::new(6.0) * t4700 * t86524 * t11944 + F::new(6.0) * t4700 * t24909 * t51906 - t4700 * t7398 * t11940, t83543);
    t86534
}
