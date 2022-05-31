
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo3214(_: S1, _: S3, _: S4, _: S7, _: S8) {}
        
        fn test3214() { foo3214(S2, S3, S2, S2, S4, S1, S2); }
    