//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1274/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1274<F: Float>(t1985: F, t26471: F, t6889: F, t6906: F, t12020: F, t120616: F, t120621: F, t120628: F, t120629: F, t120633: F, t120641: F, t1323: F, t2016: F, t22670: F, t26224: F, t26225: F, t26481: F, t31117: F, t31189: F, t32686: F, t32690: F, t32726: F, t3758: F, t3882: F, t5215: F, t5325: F, t5326: F, t568: F, t7729: F, t8485: F, t91441: F) -> F {
    let t120649 = F::new(0.16449340668482264365e-1) * t1985 * t6889 * t6906 * t26471;
    let t120652 = -F::new(6.0) * t12020 * t26224 * t5325 * t8485 + t1323 * t32726 * t568 - F::new(12.0) * t26224 * t26225 * t26481 - F::new(2.0) * t2016 * t91441 + F::new(4.0) * t22670 * t7729 - F::new(6.0) * t31117 * t5215 + F::new(2.0) * t31189 * t5326 + F::new(2.0) * t32686 * t3882 - F::new(6.0) * t32690 * t3758 - t120616 - t120621 + t120628 + t120629 + t120633 - t120641 - t120649;
    t120652
}
