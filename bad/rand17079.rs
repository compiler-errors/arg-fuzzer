
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo17079(_: S5, _: S1, _: S1, _: S3, _: S4, _: S2) {}
        
        fn test17079() { foo17079(S6, S3, S0, S0, S1, S2, S3); }
    