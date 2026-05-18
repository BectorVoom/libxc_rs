//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1269/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1269<F: Float>(t111: F, t34228: F, t119824: F, t119826: F, t119830: F, t119831: F, t119835: F, t119837: F, t119839: F, t119841: F, t119844: F, t119850: F, t119852: F, t119856: F, t119858: F, t123027: F, t123028: F, t27863: F, t5361: F, t672: F, t7271: F, t8916: F) -> (F, F) {
    let t125100 = t34228 * t111;
    let t125103 = -F::new(2.0) * t125100 * t672 - F::new(4.0) * t27863 * t7271 + t5361 * t8916 - t119824 - t119826 - t119830 + t119831 + t119835 - t119837 - t119839 - t119841 - t119844 - t119850 - t119852 - t119856 + t119858 - F::new(2.0) * t123027 + F::new(6.0) * t123028;
    (t125100, t125103)
}
