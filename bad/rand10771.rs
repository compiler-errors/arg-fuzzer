
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo10771(_: S4, _: S5, _: S6, _: S7, _: S8) {}
        
        fn test10771() { foo10771(S5, S1, S4, S6, S2, S7, S3); }
    