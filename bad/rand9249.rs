
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo9249(_: S6, _: S5, _: S3, _: S7, _: S2, _: S1, _: S6) {}
        
        fn test9249() { foo9249(S1, S5, S4, S6, S5, S7, S1); }
    