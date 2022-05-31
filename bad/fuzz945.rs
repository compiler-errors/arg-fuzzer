
struct S1;
struct S2;
struct S3;
struct S4;
struct S5;
struct S6;
struct S7;
struct S8;

fn foo945(_: S8, _: S6) {}

fn test945() {
    foo945(S2, S8, S3);
}
