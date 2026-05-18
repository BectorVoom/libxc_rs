//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1124/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1124<F: Float>(t265: F, t504: F, t27421: F, t27757: F, t27797: F, t27832: F, t3640: F, t8090: F, t1254: F, t1763: F, t1256: F, t193: F, t24905: F, t24909: F, t25882: F, t336: F, t4700: F, t5091: F, t7398: F) -> F {
    let t505 = t265 < t504;
    let t27834 = t27421 + t27757 + t27797 + t27832;
    let t27838 = t8090 * t3640;
    let t27843 = t1763 * t1254;
    let t27850 = piecewise3::<f64>(t505, t1256 * t193 * t27834 * t336 - t1254 * t27838 * t4700 - t1763 * t24905 * t4700 + F::new(2.0) * t24909 * t27843 * t4700 - t4700 * t5091 * t7398, t25882);
    t27850
}
