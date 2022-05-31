
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo9543(_: S7, _: S7, _: S1, _: S1) {}
        
        fn test9543() { foo9543(S5, S1, S4, S8, S6, S7, S2, S3); }
    