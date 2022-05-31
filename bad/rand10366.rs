
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo10366(_: S5, _: S7, _: S6, _: S3, _: S2, _: S0) {}
        
        fn test10366() { foo10366(S2, S3, S5, S6); }
    