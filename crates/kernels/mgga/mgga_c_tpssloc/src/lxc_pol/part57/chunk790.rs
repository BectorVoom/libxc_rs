//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 790/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk790<F: Float>(t3: F, t5398: F, t1933: F, t1618: F, t1622: F, t1937: F, t23447: F, t23537: F, t23541: F, t25577: F, t25580: F, t25598: F, t25616: F, t25618: F, t25625: F, t25629: F, t25645: F, t5857: F, t5861: F, t5869: F, t5875: F, t5880: F, t6755: F, t6765: F, t7583: F) -> F {
    let t28525 = t3 * t5398;
    let t28526 = t1933 * t28525;
    let t28550 = F::new(0.10093189023535097714e-3) * t28526 * t1937 - F::new(0.20186378047070195428e-3) * t25645 * t7583 + t25598 / F::new(432.0) + t25577 * t1618 / F::new(768.0) + t25580 * t1622 / F::new(1152.0) + t6755 * t5869 / F::new(1536.0) + t23537 * t5875 / F::new(768.0) - t23541 * t5880 / F::new(1536.0) + t6765 * t5857 / F::new(2304.0) + F::new(5.0) / F::new(6912.0) * t6765 * t5861 - t23447 + t25616 / F::new(1728.0) + t25618 / F::new(1152.0) + t25625 / F::new(1152.0) + F::new(0.20186378047070195428e-3) * t25629;
    t28550
}
