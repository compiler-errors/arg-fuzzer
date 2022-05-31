
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2454(_: S2, _: S3) {}
        
        fn test2454() { foo2454(S7, S6, S2, S5); }
    