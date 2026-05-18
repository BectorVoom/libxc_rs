//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1453/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1453<F: Float>(t109461: F, t109493: F, t109528: F, t109553: F, t109593: F, t109627: F, t109661: F, t109694: F, t109535: F, t1887: F, t103515: F, t103694: F, t103881: F, t104469: F, t104480: F, t109418: F, t1653: F, t1729: F, t1735: F, t2149: F, t2152: F, t21762: F, t22114: F, t24776: F, t24812: F, t24821: F, t27406: F, t27496: F, t29678: F, t29763: F, t29773: F, t3610: F, t3612: F, t470: F, t493: F, t7283: F, t7362: F, t7363: F, t8078: F, t86037: F, t95768: F) -> (F, F, F) {
    let t109697 = t109461 + t109493 + t109528 + t109553 + t109593 + t109627 + t109661 + t109694;
    let t109722 = t109535 * t1887;
    let t109732 = t470 * t493 * t109697 + F::new(3.0) * t1729 * t29773 + t22114 * t2152 + F::new(0.14621636149762012769e-1) * t95768 + F::new(0.43864908449286038307e-1) * t27406 * t29763 + F::new(0.21932454224643019154e-1) * t7283 * t24776 * t7363 * t21762 + F::new(0.54831135561607547883e-2) * t104469 - F::new(0.82246703342411321826e-2) * t7283 * t7362 * t103881 * t1653 + F::new(6.0) * t3610 * t109418 * t3612 + F::new(0.82246703342411321826e-2) * t86037 * t103694 * t24821 * t1653 - F::new(0.3752886611772249944e0) * t109722 * t2149 + F::new(0.24125699647107321069e0) * t29678 * t8078 - F::new(0.24674011002723396548e-1) * t24812 * t27496 * t103515 * t1735 + F::new(0.82246703342411321826e-2) * t104480;
    (t109697, t109722, t109732)
}
