//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1170/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1170<F: Float>(t11714: F, t476: F, t42341: F, t44696: F, t3508: F, t23508: F, t3502: F, t1209: F, t475: F, t43763: F, t44620: F, t11545: F, t43791: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t44722 = F::new(1.0) / t11714 / t476;
    let t44724 = t44696 * t42341 * t44722;
    let t44725 = t3508 * t3508;
    let t44726 = t23508 * t44725;
    let t44753 = t44696 * t42341 * t3502;
    let t44754 = t23508 * t3508;
    let t44785 = t44696 * t42341 * t1209;
    let t44786 = t23508 * t475;
    let t44805 = t44620 * t43763;
    let t44817 = t11545 * t43791;
    (t44722, t44724, t44725, t44726, t44753, t44754, t44785, t44786, t44805, t44817)
}
