//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1456/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1456<F: Float>(t11714: F, t476: F, t42341: F, t44696: F, t3508: F, t23508: F, t11883: F, t3493: F, t11889: F, t11620: F, t11638: F, t11639: F, t11877: F, t11881: F, t11888: F, t11893: F, t11904: F, t11914: F, t11915: F, t1235: F, t1244: F, t1246: F, t1247: F, t3610: F, t3611: F, t3617: F, t3624: F, t3625: F, t44673: F, t44700: F, t44707: F, t44710: F, t5068: F) -> (F, F, F) {
    let t44722 = F::new(1.0) / t11714 / t476;
    let t44724 = t44696 * t42341 * t44722;
    let t44725 = t3508 * t3508;
    let t44726 = t23508 * t44725;
    let t44730 = t11883 * t3493;
    let t44741 = t11889 * t3493;
    let t44748 = F::new(4.0) * t11638 * t1235 * t1244 * t1246 + F::new(24.0) * t11620 * t3610 * t5068 + F::new(8.0) * t11639 * t3610 * t5068 + F::new(36.0) * t11881 * t3611 * t44730 - F::new(36.0) * t11888 * t3611 * t44741 + F::new(4.0) * t11914 * t11915 * t44673 - F::new(6.0) * t3624 * t3625 * t44710 + F::new(24.0) * t44700 * t44724 * t44726 + F::new(12.0) * t11877 * t3617 + F::new(24.0) * t11893 * t11904 + F::new(4.0) * t1247 * t44707;
    (t44722, t44725, t44748)
}
