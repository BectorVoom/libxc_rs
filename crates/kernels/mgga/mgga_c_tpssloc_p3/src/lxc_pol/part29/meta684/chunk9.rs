//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2335/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2335<F: Float>(t27834: F, t3640: F, t11947: F, t8090: F, t1254: F, t1256: F, t15834: F, t1763: F, t193: F, t24905: F, t24909: F, t27838: F, t27843: F, t336: F, t3633: F, t3637: F, t4700: F, t5091: F, t64447: F, t7398: F, t86513: F, t86517: F, t86524: F, t94341: F, t94385: F, t94428: F, t94464: F, t94498: F, t94530: F, t94564: F, t94605: F, t94637: F, t94673: F, t94698: F, t94734: F, t94770: F, t95844: F, t95876: F, t95913: F) -> F {
    let t95921 = t27834 * t3640;
    let t95925 = t8090 * t11947;
    let t95952 = t193 * t336 * (t94341 + t94385 + t94428 + t94464 + t94498 + t94530 + t94564 + t94605 + t94637 + t94673 + t94698 + t94734 + t94770 + t95844 + t95876 + t95913) * t1256 - F::cast_from(2.0_f64) * t4700 * t95921 * t1254 + F::cast_from(2.0_f64) * t4700 * t95925 * t3637 - t4700 * t27838 * t3633 - t4700 * t86513 * t1763 + F::cast_from(4.0_f64) * t4700 * t86517 * t27843 - F::cast_from(2.0_f64) * t4700 * t24905 * t5091 - F::cast_from(6.0_f64) * t4700 * t86524 * t1763 * t3637 + F::cast_from(4.0_f64) * t4700 * t24909 * t64447 + F::cast_from(2.0_f64) * t4700 * t24909 * t1763 * t3633 - t4700 * t7398 * t15834;
    t95952
}
