
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo9022(_: S1, _: S2, _: S5, _: S8) {}
        
        fn test9022() { foo9022(S7, S7, S0, S2, S4); }
    