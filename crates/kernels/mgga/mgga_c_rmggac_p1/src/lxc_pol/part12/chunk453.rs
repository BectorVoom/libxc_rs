//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 453/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk453<F: Float>(t1570: F, t1574: F, t309: F, t3901: F, t4858: F, t4861: F, t4862: F, t4865: F, t4868: F, t4871: F, t4879: F, t4882: F, t4883: F, t4886: F, t4889: F, t4892: F, t538: F, t544: F, t804: F, t822: F, t826: F, t87: F, t98: F) -> F {
    let t4895 = F::new(400.0) / F::new(27.0) * t804 * t538 - F::new(200.0) / F::new(27.0) * t309 * t1570 - F::new(100.0) / F::new(9.0) * t309 * t1574 - F::new(20.0) / F::new(27.0) * t87 * t4858 + F::new(40.0) / F::new(9.0) * t4861 * t4862 + F::new(20.0) / F::new(9.0) * t87 * t4865 + F::new(10.0) / F::new(3.0) * t87 * t4868 - F::new(10.0) * t87 * t4871 - F::new(100.0) / F::new(27.0) * t544 * t822 - F::new(50.0) / F::new(9.0) * t544 * t826 - F::new(20.0) / F::new(27.0) * t98 * t4879 - F::new(40.0) / F::new(9.0) * t4882 * t4883 + F::new(20.0) / F::new(9.0) * t98 * t4886 - F::new(10.0) / F::new(3.0) * t98 * t4889 + F::new(10.0) * t98 * t4892 + t3901;
    t4895
}
