//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1657/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1657<F: Float>(t1731: F, t3540: F, t1222: F, t4961: F, t1743: F, t3566: F, t11692: F, t1174: F, t11834: F, t15686: F, t15691: F, t15699: F, t15704: F, t15710: F, t15714: F, t15717: F, t3552: F, t3557: F, t3562: F, t3577: F, t488: F, t4889: F) -> F {
    let t15719 = t1731 * t3540;
    let t15722 = t4961 * t1222 / F::new(432.0);
    let t15723 = t3566 * t1743;
    let t15726 = t1174 * t15686 / F::new(36.0) - t15691 + t4889 * t3552 / F::new(108.0) + t4889 * t3557 / F::new(54.0) - t4889 * t3562 / F::new(81.0) + t15699 + t11692 * t15704 / F::new(2304.0) - t3577 * t15710 / F::new(1152.0) + t11834 + F::new(5.0) / F::new(13824.0) * t3577 * t15714 + t15717 / F::new(2592.0) - t15719 / F::new(13824.0) - t15722 - t15723 * t488 / F::new(576.0);
    t15726
}
