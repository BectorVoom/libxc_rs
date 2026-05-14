//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 885/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk885<F: Float>(t5166: F, t588: F, t11981: F, t2528: F, t5154: F, t172: F, t5151: F, t763: F, t2535: F, t592: F, t118: F, t1787: F, t2375: F, t12045: F, t12052: F, t12054: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t15880 = 8.0 * t588 * t5166;
    let t15889 = 32.0 * t11981;
    let t15890 = t5154 * t2528;
    let t15892 = t5151 * t172;
    let t15894 = 0.11696447245269292414e1 * t15892 * t763;
    let t15895 = t5154 * t2535;
    let t15898 = 8.0 * t592 * t5166;
    let t15908 = t1787 * t118;
    let t15909 = t15908 * t2375;
    let t15911 = 48.0 * t12045;
    let t15916 = 12.0 * t12052;
    let t15917 = 80.0 * t12054;
    (t15880, t15889, t15890, t15894, t15895, t15898, t15909, t15911, t15916, t15917)
}
