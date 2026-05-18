//! MGGA_X_BR89_EXPLICIT exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_br89_explicit.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_x_br89_explicit_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = 1.0 <= zeta_threshold;
        let t5 = zeta_threshold - 1.0;
        let t7 = piecewise5::<f64>(t4, t5, t4, -t5, 0.0);
        let t8 = 1.0 + t7;
        let t10 = pow_1_3::<f64>(zeta_threshold);
        let t12 = pow_1_3::<f64>(t8);
        let t14 = piecewise3::<f64>(t8 <= zeta_threshold, t10 * zeta_threshold, t12 * t8);
        let t15 = pow_1_3::<f64>(rho[ip]);
        let t16 = t14 * t15;
        let t18 = pow_1_3::<f64>(1.0 / M_PI);
        let t19 = 1.0 / t18;
        let t20 = t16 * t19;
        let t21 = M_CBRT4;
        let t22 = M_CBRTPI;
        let t23 = t22 * t22;
        let t24 = M_CBRT2;
        let t25 = t24 * t24;
        let t26 = t15 * t15;
        let t28 = 1.0 / t26 / rho[ip];
        let t31 = param_gamma * tau[ip];
        let t34 = param_gamma * sigma[ip];
        let t35 = rho[ip] * rho[ip];
        let t37 = 1.0 / t26 / t35;
        let t41 = f64::abs(lapl[ip] * t28 / 2.0 - 2.0 * t31 * t28 + t34 * t37 / 4.0);
        let t44 = t25 * t41 / 3.0 < 0.5e-12;
        let t45 = lapl[ip] * t25;
        let t48 = t25 * t28;
        let t51 = t25 * t37;
        let t54 = t45 * t28 / 6.0 - 2.0 / 3.0 * t31 * t48 + t34 * t51 / 12.0;
        let t55 = 0.0 < t54;
        let t56 = piecewise3::<f64>(t55, 0.5e-12, -0.5e-12);
        let t57 = piecewise3::<f64>(t44, t56, t54);
        let t60 = 2.0 / 3.0 * t23 / t57;
        let t61 = t60 <= 0.0;
        let t62 = -0.5e-12 < t60;
        let t63 = piecewise3::<f64>(t62, -0.5e-12, t60);
        let t65 = 0.1525525181200953e1 * t63 + 0.4576575543602858e0;
        let t66 = f64::atan(t65);
        let t67 = -t66 + 0.4292036732051034e0;
        let t69 = t63 * t63;
        let t71 = t69 * t63;
        let t73 = t69 * t69;
        let t75 = t73 * t63;
        let t77 = 0.7566445420735584e0 - 0.2636397787137096e1 * t63 + 0.5474515996423288e1 * t69 - 0.1265730812710829e2 * t71 + 0.4125058472512136e1 * t73 - 0.3042513395716384e2 * t75;
        let t78 = t67 * t77;
        let t84 = 0.4771976183772063e0 - 0.1779981349455627e1 * t63 + 0.3843384186230215e1 * t69 - 0.9591205088051849e1 * t71 + 0.2173018028591672e1 * t73 - 0.3042513385160366e2 * t75;
        let t85 = 1.0 / t84;
        let t87 = 0.5e-12 < t60;
        let t88 = piecewise3::<f64>(t87, t60, 0.5e-12);
        let t90 = f64::ln(1.0 / (0.2085749716493756e1 * t88) + f64::sqrt(pow_2::<f64>(1.0 / (0.2085749716493756e1 * t88)) + 1.0));
        let t91 = t90 + 2.0;
        let t93 = t88 * t88;
        let t95 = t93 * t88;
        let t97 = t93 * t93;
        let t99 = t97 * t88;
        let t101 = 0.4435009886795587e-4 + 0.5812865360445791e0 * t88 + 0.6674276451594061e2 * t93 + 0.4342678089722977e3 * t95 + 0.8247765766052239e3 * t97 + 0.1657965273158212e4 * t99;
        let t102 = t91 * t101;
        let t108 = 0.3347285060926091e-4 + 0.4791793102397135e0 * t88 + 0.6239226833857424e2 * t93 + 0.4631481642793812e3 * t95 + 0.7852360350104029e3 * t97 + 0.1657962968223273e4 * t99;
        let t109 = 1.0 / t108;
        let t111 = piecewise3::<f64>(t61, t78 * t85, t102 * t109);
        let t113 = f64::exp(t111 / 3.0);
        let t114 = t21 * t113;
        let t115 = f64::exp(-t111);
        let t117 = 1.0 + t111 / 2.0;
        let t118 = t115 * t117;
        let t119 = 1.0 - t118;
        let t120 = 1.0 / t111;
        let t121 = t119 * t120;
        let t122 = t114 * t121;
        let t125 = piecewise3::<f64>(t3, 0.0, -t20 * t122 / 4.0);
        let tzk0 = 2.0 * t125;
        zk[ip] += tzk0;
    }
}
