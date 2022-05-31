
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo11494(_: S2, _: S6, _: S0, _: S0, _: S0, _: S2) {}
        
        fn test11494() { foo11494(S1, S3, S4, S5, S2, S7, S6, S8); }
    