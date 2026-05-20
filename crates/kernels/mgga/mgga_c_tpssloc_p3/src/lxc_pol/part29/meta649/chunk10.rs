//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2168/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2168<F: Float>(t1081: F, t4255: F, t870: F, t23788: F, t58071: F, t86706: F, t1649: F, t2745: F, t25927: F, t86713: F, t2379: F, t1877: F, t1915: F, t22959: F, t23789: F, t23792: F, t25013: F, t2522: F, t25372: F, t4314: F, t6670: F, t6848: F, t7541: F, t86736: F, t86836: F, t89837: F, t89840: F, t89843: F, t89846: F, t89850: F) -> F {
    let t89859 = t870 * t1081 * t4255;
    let t89862 = t23788 * t58071;
    let t89865 = t23788 * t86706;
    let t89868 = t1649 * t2745;
    let t89872 = t25927 * t86713;
    let t89874 = t1649 * t2379;
    let t89880 = -F::new(3.0) / F::new(2.0) * t22959 * t89837 - F::new(3.0) / F::new(2.0) * t22959 * t89840 + F::new(3.0) * t25013 * t89843 + F::new(2.0) * t25372 * t89846 + F::new(2.0) * t25372 * t89850 - F::new(3.0) * t86736 * t23789 + F::new(3.0) * t2522 * t7541 * t23792 + F::new(6.0) * t25013 * t89859 - F::new(3.0) * t22959 * t89862 - F::new(3.0) * t25013 * t89865 - t1877 * t6670 * t89868 / F::new(2.0) + t25372 * t89872 + F::new(3.0) * t4314 * t1915 * t89874 - t1877 * t86836 * t6848;
    t89880
}
