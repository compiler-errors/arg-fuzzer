
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2574(_: S1, _: S7) {}
        
        fn test2574() { foo2574(S6, S4, S1, S3, S5); }
    