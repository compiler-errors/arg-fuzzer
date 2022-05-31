
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo3614(_: S3, _: S5) {}
        
        fn test3614() { foo3614(S2, S3, S6, S7); }
    