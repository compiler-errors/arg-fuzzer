
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo226(_: S8, _: S2) {}
        
        fn test226() { foo226(S4, S8, S6); }
    