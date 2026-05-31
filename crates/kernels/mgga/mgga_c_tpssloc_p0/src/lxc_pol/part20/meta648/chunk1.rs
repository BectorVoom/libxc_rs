//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2380/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2380<F: Float>(t10813: F, t4433: F, t10743: F, t10765: F, t10771: F, t10805: F, t10811: F, t14266: F, t14328: F, t14432: F, t14435: F, t14436: F, t14442: F, t1569: F, t2861: F, t2862: F, t2880: F, t2881: F, t2886: F, t2888: F, t2889: F, t311: F, t41984: F, t42154: F, t47791: F, t48747: F, t48750: F, t48765: F, t48789: F, t48813: F, t48833: F, t931: F) -> F {
    let t48854 = t4433 * t10813;
    let t48861 = t48747 + t48750 + F::cast_from(3.0_f64) * t14266 * t2881 + F::cast_from(0.96491876992155210402e2_f64) * t48789 * t2889 - F::cast_from(0.19751673498613801407e-1_f64) * t47791 - F::cast_from(0.310907e-1_f64) * (t48813 + t48833) * t311 + t48765 - F::cast_from(2.0_f64) * t2861 * t1569 * t10805 - F::cast_from(0.57895126195293126242e3_f64) * t41984 * t14432 - F::cast_from(0.24828486201251232145e5_f64) * t42154 * t14442 * t10743 + F::cast_from(0.19298375398431042081e3_f64) * t10765 * t14436 + F::cast_from(0.96491876992155210402e2_f64) * t2886 * t14328 * t2888 * t931 + F::cast_from(0.96491876992155210402e2_f64) * t2886 * t14435 * t2880 + F::cast_from(0.6207121550312808036e4_f64) * t10811 * t48854 * t2862 - F::cast_from(24.0_f64) * t10771 * t1569 * t10743;
    t48861
}
