//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 795/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk795<F: Float>(t3: F, t5398: F, t1915: F, t5527: F, t1484: F, t1530: F, t1877: F, t193: F, t202: F, t23295: F, t2522: F, t25358: F, t28248: F, t28447: F, t4314: F, t5544: F, t5660: F, t5664: F, t6670: F, t7541: F, t870: F) -> (F, F) {
    let t28525 = t3 * t5398;
    let t28732 = t1915 * t5527;
    let t28755 = t193 * t202 * t28447 * t870 + F::new(6.0) * t1484 * t2522 * t7541 - F::new(2.0) * t1530 * t1877 * t25358 + F::new(2.0) * t1877 * t23295 * t5664 - t1877 * t5660 * t6670 + F::new(3.0) * t1915 * t2522 * t5544 - F::new(6.0) * t2522 * t28248 * t6670 + F::new(6.0) * t28732 * t4314;
    (t28525, t28755)
}
