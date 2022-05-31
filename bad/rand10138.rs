
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo10138(_: S2, _: S3, _: S4, _: S8) {}
        
        fn test10138() { foo10138(S1, S2, S4, S5, S6, S7); }
    